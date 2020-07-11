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
        players.set(message.author.id,new Player(message.author.username));
        
        console.log("Created user " + message.author.username + " id:" + message.author.id);
        console.log(players);
    }else if(command === 'gold'){
        let player = players.get(message.author.id);
        message.channel.send(player.name + " : " + player.gold + " gold");
    }





});

