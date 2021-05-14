const fetch = require('node-fetch');
module.exports = {
    name : "joke",
    descrption : ": tells a joke!",
    execute(message,args,players){
        const url = "https://icanhazdadjoke.com/"
        fetch(url, {method : "GET", headers : {'Accept': 'application/json'}})
        .then(res => res.json())
        .then(json => message.channel.send(json.joke))
        .catch(err => console.log(err))
    }
}