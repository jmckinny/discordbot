module.exports={
    name: 'coinflip',
    description: '[gold]',
    execute(message,args,players){
        let player = players.get(message.author.id);
        console.log(args[0]);
        if(args[0] == null){
            if(Math.ceil(Math.random() * 2) === 1){
                message.reply("Winner!");
            }else{
                message.reply("Loser!");
            }
        }else{
            const amount = parseInt(args[0]);
	    if(isNaN(amount) || amount <= 0){
                message.reply("Invalid gold amount");
		return;
	    }
            if(amount > player.gold){
                message.reply("You don't have that much gold!");
                return;
            }
            if(Math.ceil(Math.random() * 2) === 1){
                message.reply(`Winner! ${player.name} wins ${amount} gold.`);
                player.addGold(amount);
            }else{
                message.reply(`Loser! ${player.name} loses ${amount} gold.`);
                player.addGold(-amount);
            }
        }
        
    }
}


