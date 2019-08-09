workflow "main" {
  on = "push"
  resolves = ["build", "deploy"]
}

action "build" {
  uses = "actions/docker/cli@master"
  args = "build -t docker.pkg.github.com/sagebind/blog/app:$GITHUB_SHA ."
}

action "master" {
  needs = ["build"]
  uses = "actions/bin/filter@b2bea07"
  args = "branch master"
}

action "registry-login" {
  uses = "actions/docker/login@76ff57a"
  env = {
    DOCKER_REGISTRY_URL = "docker.pkg.github.com"
  }
  secrets = ["DOCKER_USERNAME", "DOCKER_PASSWORD"]
}

action "push-image" {
  needs = ["build", "registry-login"]
  uses = "actions/docker/cli@master"
  args = "push docker.pkg.github.com/sagebind/blog/app:$GITHUB_SHA"
}

action "deployment-config" {
  uses = "actions/bin/sh@master"
  args = ["sed -i s/:latest/:$GITHUB_SHA/ $GITHUB_WORKSPACE/config/deployment.yaml"]
}

action "kubeconfig" {
  needs = ["master"]
  uses = "digitalocean/action-doctl@master"
  secrets = ["DIGITALOCEAN_ACCESS_TOKEN"]
  args = ["kubernetes cluster kubeconfig show nyc1 > $HOME/.kubeconfig"]
}

action "deploy" {
  needs = ["kubeconfig", "push-image", "deployment-config"]
  uses = "docker://lachlanevenson/k8s-kubectl"
  runs = "sh -l -c"
  args = ["kubectl --kubeconfig=$HOME/.kubeconfig apply -f $GITHUB_WORKSPACE/config/deployment.yaml"]
}
