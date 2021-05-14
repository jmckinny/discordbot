module.exports = {
    name: 'playing',
    description: ': lists active players',
    execute(message,args,players){
        message.channel.send(`There are ${players.size} active players`);
        if(players.size > 0){
            let str = ""
            players.forEach(element => {
                str += `${element.name} : ${element.gold} gold \n`;
            });
            
            message.channel.send(str);
        }
        
    }
}

