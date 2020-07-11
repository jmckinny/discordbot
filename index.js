const Discord = require('discord.js');
const Player = require('./player.js')
const { prefix, token } = require('./config.json');
const client = new Discord.Client();
// const deathroll = require('./deathroll');

let players = new Map();

client.once('ready', () => {
    console.log("READY!");
})

client.login(token);



client.on('message', message =>{
    if(!message.content.startsWith(prefix) || message.author.bot) return;

    const args = message.content.slice(prefix.length).split(/ +/);
    const command = args.shift().toLowerCase();

    if(command === 'ping'){
        message.channel.send('Pong!')
    }else if(command ==='help'){
        const helpmssg = "Use " + prefix + " followed by a command to use the bot"
        message.channel.send(helpmssg);
    }else if(command === 'join'){ //Adds the user to the active players
        if(players.has(message.author.id)){
            message.channel.send("Error " + players.get(message.author.id).mention + " already has joined")
        }else{
            players.set(message.author.id,new Player(message.author.id,message.author.username));
        
            console.log("Created user " + message.author.username + " id:" + message.author.id);
            console.log(players);
        }
        
    }else if(command === 'gold'){//Gets the gold of the player
        if(args[0] == null){
            let player = players.get(message.author.id);
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
        
    }else if(command === 'coinflip'){
        let player = players.get(message.author.id);
        if(Math.ceil(Math.random() * 2) === 1){
            message.channel.send("Winner!");
        }else{
            message.channel.send("Loser!");
        }
    }





});

