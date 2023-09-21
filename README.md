# Welcome to the id assigner

- [What it does](#description)
- [How to use it](#usage)
- [Whats left to do](#todo)

## Description

ID assigner is a daemon that runs on the head node of a slurm cluster.
Every node upon waking up, systemd service calls to the API, and gets a id for it's hostname.
Upon shutdown, another service calls to the API to remove id from "used" list, so it can be reassigned.

## Usage

For now, `sudo` permissions are required only for the first run to place slqlite db into `/var/service/assigner` folder.

Ignoring test endpoints, requesting 
- `/api/ip/get` will return an node struct in json form (id:i32, ip:String, created_at:DateTime).
- `/api/ip/del` requires Json with field "ip" with the ip of the node that requests it's node id to be removed from usage. It will return Json with the node point that was removed.

For now API checks that node that is being removed is indeed the one who is asking to be removed (it's an open api for now, no authentication, dont want someone to accidentaly or intentionally to remove the whole cluster from usage).


## Todo

- [ ] Add systemd services to the repo
- [ ] Build as debian package
- [ ] Add some authentication (maybe the MUNGE key?)
