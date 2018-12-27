workflow "Main" {
  on = "push"
  resolves = ["Push"]
}

action "Build" {
  uses = "actions/docker/cli@76ff57a"
  args = "build -t sagebind/blog ."
}

action "Master" {
  needs = ["Build"]
  uses = "actions/bin/filter@b2bea07"
  args = "branch master"
}

action "Registry login" {
  needs = ["Master"]
  uses = "actions/docker/login@76ff57a"
  secrets = ["DOCKER_USERNAME", "DOCKER_PASSWORD"]
}

action "Push" {
  needs = ["Registry login"]
  uses = "actions/docker/cli@76ff57a"
  args = "push sagebind/blog"
}
