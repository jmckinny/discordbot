const Player = require('../player.js');
module.exports={
    name:'join',
    descritpion: 'adds player to list of active players',
    execute(message,args,players){
        players.insert(new Player(message.author.id,message.author.username));
        players.find({}).then((docs) => {
            console.log(docs);
        });
    }
}