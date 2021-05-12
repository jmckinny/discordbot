let isNaN = (maybeNaN) => maybeNaN != maybeNaN;
module.exports = {
    name: 'deathroll',
    description: 'starts a deathroll agaisnt a given player',
    execute(message, args, players) {
        let member = message.mentions.members.first();
        let amount = parseInt(args[1]);
        if (args[1] == null) {
            amount = 5;
        }
        if (!players.has(message.author.id) || !players.has(member.id)) {
            message.reply("one of the players has not joined!");
        } else {
            let player1 = players.get(message.author.id);
            let player2 = players.get(member.id);
            let gameover = false;
	    if (isNaN(amount) || amount <= 1){
            message.reply("Invalid gold amount");
		return;
	    }
            if(amount > player1.gold || amount > player2.gold){
                message.channel.send("Not all users have the required gold!");
                return;
            }

            const quitFilter = response => {
                return ((response.content === 'yes' || response.content === 'no') && (response.author.id === player2.id));
            }
            message.channel.send(`${player1.name} challanges ${player2.name} to a deathroll for ${amount} gold.  Do you accept? (yes or no)`).then(() => {
                message.channel.awaitMessages(quitFilter, { max: 1, time: 30000, errors: ['time'] })
                    .then(collected => {
                        if (collected.first().content === 'no') {
                            gameover = true;
                            message.reply("deathroll declined");
                        } else {
                            message.reply("deathroll accepted!");

                            ///MAIN GAME LOOP/////////////////
                            let activePlayer = player1;
                            let winnings = amount;
                            let winner,loser;
                            while (!gameover) {
                                message.channel.send(`${activePlayer.name}'s roll at ${amount}`);
                                amount = activePlayer.roll(amount);
                                message.channel.send(`${activePlayer.name} rolled ${amount}`);
                                if (amount === 1) {
                                    gameover = true;
                                    message.channel.send(`${activePlayer.name} loses!`);
                                    winner = activePlayer === player1 ? player2 : player1;
                                    loser = activePlayer;
                                }
                                activePlayer = activePlayer === player1 ? player2 : player1;
                            }
                            /////END OF GAME LOOP//////
                            message.channel.send(`${winner.name} won ${winnings} gold!`);
                            winner.addGold(winnings);
                            message.channel.send(`${loser.name} lost ${winnings} gold!`);
                            loser.addGold(-winnings);
                        }
                    })
                    .catch(collected => {
                        message.channel.send("Timeout error! Deathroll cancelled.");
                        return;
                    })
            })






        }
    }

}

