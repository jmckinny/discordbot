module.exports = {
    name: 'ping',
    description: 'Ping!',
    execute(message,args,players){
        message.channel.send('Pong!')
    }
}