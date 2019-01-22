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
  uses = "sagebind/actions/swarm-deploy@master"
  needs = ["Push image"]
  secrets = ["DOCKER_SSH_KEY"]
  env = {
    DOCKER_HOST = "ssh://root@45.55.121.98"
  }
  args = "--prune --compose-file deploy/default.yml stephencoakley"
}
