const Player = require('../player.js');
module.exports={
    name:'join',
    description: ': adds player to list of active players',
    execute(message,args,players){
        if(players.has(message.author.id)){
            message.channel.send("Error " + players.get(message.author.id).mention + " already has joined")
        }else{
            players.set(message.author.id,new Player(message.author.id,message.author.username));
        
            console.log("Created user " + message.author.username + " id:" + message.author.id);
            message.reply(`created user`);
        }
    }
}