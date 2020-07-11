const Discord = require('discord.js');
const { prefix, token } = require('./config.json');
const client = new Discord.Client();
// const deathroll = require('./deathroll');

let players = [];

client.once('ready', () => {
    console.log("READY!")
})

client.login(token);

client.on('message', message =>{
    if(!message.content.startsWith(prefix) || message.author.bot) return;

    const args = message.content.slice(prefix.length).split(/ +/);
    const command = args.shift().toLowerCase();

    if(command === 'ping'){
        message.channel.send('Pong!')
    }else if(command ==='help'){
        
    }





});

