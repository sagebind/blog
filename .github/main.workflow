workflow "Main" {
  on = "push"
  resolves = [
    "Build",
    "Master",
    "Registry login",
    "Push",
  ]
}

action "Build" {
  uses = "docker://docker/compose:1.23.2"
  args = "build"
}

action "Master" {
  uses = "actions/bin/filter@b2bea07"
  needs = ["Build"]
  args = "branch master"
}

action "Registry login" {
  uses = "actions/docker/login@76ff57a"
  needs = ["Master"]
  secrets = ["DOCKER_USERNAME", "DOCKER_PASSWORD"]
}

action "Push" {
  uses = "docker://docker/compose:1.23.2"
  needs = ["Registry login"]
  args = "push"
}
