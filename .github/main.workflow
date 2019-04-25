workflow "main" {
  on = "push"
  resolves = ["deploy"]
}

action "build" {
  uses = "actions/docker/cli@76ff57a"
  args = "build -t sagebind/blog ."
}

action "master" {
  uses = "actions/bin/filter@b2bea07"
  args = "branch master"
}

action "registry-login" {
  needs = ["build", "master"]
  uses = "actions/docker/login@76ff57a"
  secrets = ["DOCKER_USERNAME", "DOCKER_PASSWORD"]
}

action "push-image" {
  needs = ["build", "registry-login"]
  uses = "actions/docker/cli@76ff57a"
  args = "push sagebind/blog"
}

action "kubeconfig" {
  needs = ["master"]
  uses = "digitalocean/action-doctl@master"
  secrets = ["DIGITALOCEAN_ACCESS_TOKEN"]
  args = ["kubernetes cluster kubeconfig show nyc1 > $HOME/.kubeconfig"]
}

action "deploy" {
  needs = ["kubeconfig", "push-image"]
  uses = "docker://lachlanevenson/k8s-kubectl"
  runs = "sh -l -c"
  args = ["kubectl --kubeconfig=$HOME/.kubeconfig apply -f $GITHUB_WORKSPACE/config/deployment.yaml"]
}
