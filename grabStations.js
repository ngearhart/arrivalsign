const fs = require('fs');

fetch("https://api.wmata.com/Rail.svc/json/jStations", {
    headers: {
    "api_key": "API_KEY"
    }
}).then(response => {
    response.json().then(data => {
        fs.writeFileSync('stations_objs.json',
            JSON.stringify(
                Object.fromEntries(data["Stations"].map(station => [station.Name, station.Code]))
            )
        )
        fs.writeFileSync('stations_list.json',
            JSON.stringify(
                data["Stations"].map(station => ({"name": station.Name, "code": station.Code}))
            )
        )
    })
})
