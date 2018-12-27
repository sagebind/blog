workflow "Main" {
  on = "push"
  resolves = [
    "Build",
#     "Master",
    "Registry login",
    "Push",
  ]
}

action "Build" {
  uses = "actions/docker/cli@76ff57a"
  args = "build -t sagebind/blog ."
}

# action "Master" {
#   uses = "actions/bin/filter@b2bea07"
#   needs = ["Build"]
#   args = "branch master"
# }

action "Registry login" {
  uses = "actions/docker/login@76ff57a"
  needs = ["Build"]
  secrets = ["DOCKER_USERNAME", "DOCKER_PASSWORD"]
}

action "Push" {
  uses = "actions/docker/cli@76ff57a"
  needs = ["Registry login"]
  args = "push sagebind/blog"
}
