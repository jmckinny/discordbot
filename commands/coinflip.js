module.exports={
    name: 'coinflip',
    descrption: 'a 50% chance to win!',
    execute(message,args,players){
        let player = players.get(message.author.id);
        if(Math.ceil(Math.random() * 2) === 1){
            message.channel.send("Winner!");
        }else{
            message.channel.send("Loser!");
        }
    }
}