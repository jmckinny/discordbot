const Discord = require('discord.js');
const { prefix, token } = require('./config.json');
const client = new Discord.Client();
// const deathroll = require('./deathroll');

let players = [];

client.once('ready', () => {
    console.log("READY!")
})

client.login(token);

class Player {
    constructor(id, name) {
        this.id = id;
        this.name = name;
        this.gold = 50;
        this.collected = new Date();
    }

    roll(number) { //Generates a random number from 1 to number
        return Math.ceil(Math.random() * number);
    }

}


client.on('message', message => {
    //console.log(message.content)
    const args = message.content.slice(prefix.length).split(' ');
    if (message.content.startsWith(`${prefix}help`)) {
        let help = "Command List:\n " + 
        "join : creates a new account for the player\n" +
        "roll {amount} : starts a game wagering the amount given\n" +
        "gold : checks the amount of money you have\n" +
        "deathroll {opponent} {amount} : starts a deathroll agaisnt the given opponent for the amount given\n" + 
        "collect : collect your daily income\n";
        message.channel.send(help);
        
    }



    if (message.content.startsWith(`${prefix}roll`)) {
        message.channel.send("Rolled " + Math.ceil(Math.random() * 10))
    }

    if (message.content.startsWith(`${prefix}players`)) {
        message.channel.send("There are " + players.length + " active players.")
        for (let i = 0; i < players.length; i++) {
            message.channel.send(players[i].name + " : " + players[i].gold + " gold");
        }
    }



    if (message.content.startsWith(`${prefix}gold`)) {
        let id = message.member.user.id;
        let player = findPlayer(id);
        if (player === null) {
            message.channel.send("Player not found in active players.  Type '$join' to join.");
        } else {
            message.channel.send(player.gold);
        }

    }


    if (message.content.startsWith(`${prefix}join`)) {
        let id = message.member.user.id;
        let name = message.member.user.tag;
        if (findPlayer(id) === null) {
            players.push(new Player(id, name));
            console.log(players)
        } else {
            message.channel.send("That player already exists");
        }

    }

    if (message.content.startsWith(`${prefix}deathroll`)) {
        const args = message.content.split(' ');
        let amount = parseInt(args[2]);
        let player1 = findPlayer(message.member.id);
        let player2 = findPlayer(message.mentions.members.first().id);
        if (player1 === null || player2 === null) {
            message.channel.send("Invalid players, type '$join' to join");
        } else {
            message.channel.send(player1.name + " challenges " + player2.name + " to deathroll for " + amount + " gold.");
            if (player1.gold < amount) {
                message.channel.send(player1.name + " does not have enough gold!");
            }else if (player2.gold < amount) {
                message.channel.send(player2.name + " does not have enough gold!");
            }else{
                message.channel.send(player2.name + " type 'y' to accept or 'n' to decline");
                client.on('message', response =>{
                    if(response.member.id === player2.id){
                        if(response.content === "y"){
                            game(player1,player2,amount,message);
                        }else{
                            response.channel.send("Roll cancelled");
                        }
                    }
                })
               
            }
           

            

        }


    }

    if(message.content.startsWith(`${prefix}collect`)){
        player = findPlayer(message.member.id);
        if(player === null){
            message.channel.send("Player not found in active players.  Type '$join' to join.")
        }else{
            let now = new Date();
            console.log(now.getTime() + " now");
            console.log(player.collected.getTime() + " collected");
            dif = now.getTime() - player.collected.getTime();
            console.log(dif);
            if(now.getHours - player.collected.getHours >= 86400000){
                message.channel.send("You collect " + "10"+ " gold");
                player.gold = player.gold + 10;

            }else{
                message.channel.send("You can collect in " + ((86400000 - dif)/3,600,000) +" hours" );
            }
            
        }
    }

})

function findPlayer(id) { //This function given a player ID returns the object if they are active or null if they are not found
    for (i = 0; i < players.length; i++) {
        if (players[i].id === id) {
            return players[i];
        }
    }
    return null;
}

function game(player1,player2,amount,message){
    let winner,loser;
    let current = amount;
    while (true) {
        let p1roll = player1.roll(current);
        message.channel.send(player1.name + " rolled " + p1roll);
        current = p1roll;

        if (current == 1) {
            message.channel.send(player2.name + " wins!");
            winner = player2;
            loser = player1;
            break;
        }

        let p2roll = player2.roll(current);
        message.channel.send(player2.name + " rolled " + p2roll);
        current = p2roll;

        if (current == 1) {
            message.channel.send(player1.name + " wins!");
            winner = player1;
            loser = player2;
            break;
        }
    }

    winner.gold = winner.gold+amount;
    loser.gold = loser.gold - amount;
}
