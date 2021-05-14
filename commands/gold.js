module.exports={
    name: 'gold',
    description: '[@player]',
    execute(message,args,players){
        if(!players.has(message.author.id)){
            message.reply("You have not joined!");
        }else{
            let player = players.get(message.author.id);
            if(args[0] == null){
                message.channel.send(player.mention + " : " + player.gold + " gold");
            }else{
                let member = message.mentions.members.first();
                if(!players.has(member.id)){
                    message.channel.send("That user does not exist");
                }else{
                    let player = players.get(member.id);
                    message.channel.send(player.mention + " : " + player.gold + " gold");
                }
            }
        }
        
    }
}