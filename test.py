import subprocess

# Register and create game as England
subprocess.run(["cargo", "run", "-p", "cli", "register", "Louis", "1234"])
subprocess.run(["cargo", "run", "-p", "cli", "create"])

# Get game id
game = input("Enter the game: ")

# England orders
subprocess.run([
    "cargo", "run", "-p", "cli", "order",
    "-o",
    '["ENG: F edi -> nth", "ENG: F lon -> eng", "ENG: A lvp -> edi"]'
])

users_dict = {
    "french": '["FRA: F bre -> mao", "FRA: A par -> bur", "FRA: A mar -> spa"]',
    "german": '["GER: F kie -> den", "GER: A ber -> kie", "GER: A mun -> ruh"]',
    "italian": '["ITA: F nap -> ion", "ITA: A rom -> apu", "ITA: A ven -> tyr"]',
    "austrian": '["AUS: F tri -> adr", "AUS: A vie -> bud", "AUS: A bud -> ser"]',
    "russian": '["RUS: F sev -> bla", "RUS: A mos -> ukr", "RUS: A war -> gal", "RUS: F stp(sc) -> bot"]',
    "turkish": '["TUR: F ank -> bla", "TUR: A con -> bul", "TUR: A smy -> con"]',
}

for user, orders in users_dict.items():
    subprocess.run(["cargo", "run", "-p", "cli", "register", user, "1234"])
    subprocess.run(["cargo", "run", "-p", "cli", "join", game])
    subprocess.run([
        "cargo", "run", "-p", "cli", "order",
        "-o",
        orders
    ])

