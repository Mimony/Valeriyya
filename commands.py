from requests import get, post, patch, delete
from json import loads
from sys import argv

url = "https://discord.com/api/v9/applications/830130301535649853/commands"
headers = { "Authorization": "ODMwMTMwMzAxNTM1NjQ5ODUz.YHCNFg.FwlkE2je_AAfw5gIn2qn0EO3Vuc" }

with open("commands.json", "r") as f:
    json = loads(f.read())

commands = get(url).json()

def create(name: str):
    post(url, headers=headers, json=json[name])

def update(name: str, id: str):
    patch(f"{url}/{id}", headers=headers, json=json[name])

def remove(name: str, id: str):
    delete(f"{url}/{id}", headers=headers, json=json[name])

if len(argv[1:]) > 0:
    for arg in argv[1:]:
        method, name = arg.split(":")

        # Add a new command
        if method == "add": print(create(name))

        # Update a command
        elif method == "up":
            command = list(filter(lambda obj: obj["name"] == name, commands))

            if len(command) == 0:
                print(create(name))
            else:
                print(update(name, command[0]["id"]))

        elif method == "del":
            command = list(filter(lambda obj:obj["name"] == name, commands))

            if len(command) != 0:
                print(remove(name, command[0]["id"]))

    print("\n\n")