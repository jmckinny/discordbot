const fetch = require('node-fetch');
module.exports={
    name : "fortune",
    description : "Tells you a fortune!",
    async execute(message,args,players){
        fetch("http://yerkee.com/api/fortune")
        .then(response => response.json()
        .then(data => message.reply(data.fortune))
        .catch(error => console.log("Error:" + error)));      
    }
}