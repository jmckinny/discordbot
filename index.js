const fs = require('fs'); //Require the filesystem interface
const Discord = require('discord.js');
const { prefix, token } = require('./config.json');
const client = new Discord.Client(); //Make client
client.commands = new Discord.Collection(); //Add the list of commands to a collection
const commandFiles = fs.readdirSync('./commands').filter(file => file.endsWith('.js')); //Pull all command files


let players = new Map();

for (const file of commandFiles) { //Loop through command files and create a hashtable of them
	const command = require(`./commands/${file}`);
	client.commands.set(command.name, command);
}

client.once('ready', () => {
    console.log("READY!");
})

client.login(token);





client.on('message', message =>{
    if(!message.content.startsWith(prefix) || message.author.bot) return;

    const args = message.content.slice(prefix.length).split(/ +/);
    const command = args.shift().toLowerCase();

    if(!client.commands.has(command)) return;

    try{
        client.commands.get(command).execute(message,args,players);
    }catch(error){
        console.error(error);
        message.reply('there was an error trying to execute that command!');
    }
    

});

