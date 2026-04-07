# installation

1. grab the latest release from here, or compile yourself. (figure this out yourself)
2. copy the binary into your desired directory, and make a file in it called ".env"
3. copy your discord token into the file as such:
```
DISCORD_TOKEN = <insert here>
```

# Commands (discord)

/help                   show info about available commands.
/help <command>         show more info about a spesific command.
/events <city>          show a list of all events for this city currently available on the fabtcg event locator.
/post <country> <city>  schedule all events from the chosen city as discord events, using data from fabtcg event locator.

## Available cities and countries

This bot does not let the user search event locator directly. Instead you get to pick from these options:

- Denmark
    - Århus
    - København
- Norway
    - Oslo
    - Stavanger
    - Drammen
    - Lillehammer
    - Bodø
- Sweden
    - Stockholm
    - Göteborg
