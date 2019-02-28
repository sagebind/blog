workflow "Main" {
  on = "push"
  resolves = ["Deploy to swarm"]
}

action "Build image" {
  uses = "actions/docker/cli@76ff57a"
  args = "build -t sagebind/blog ."
}

action "Master" {
  needs = ["Build image"]
  uses = "actions/bin/filter@b2bea07"
  args = "branch master"
}

action "Registry login" {
  needs = ["Master"]
  uses = "actions/docker/login@76ff57a"
  secrets = ["DOCKER_USERNAME", "DOCKER_PASSWORD"]
}

action "Push image" {
  needs = ["Registry login"]
  uses = "actions/docker/cli@76ff57a"
  args = "push sagebind/blog"
}

action "Deploy to swarm" {
  uses = "sagebind/docker-swarm-deploy-action@master"
  needs = ["Push image"]
  env = {
    DOCKER_REMOTE_HOST = "ssh://root@45.55.121.98"
  }
  secrets = ["DOCKER_SSH_PRIVATE_KEY", "DOCKER_SSH_PUBLIC_KEY"]
  args = "stack deploy --with-registry-auth --prune --compose-file deploy/prod.yaml stephencoakley"
}
