const fetch = require('node-fetch');
const shuffle = require('shuffle-array');
module.exports = {
    name: 'trivia',
    descrption: 'answer a trivia question for gold!',
    async execute(message, args, players) {
        let response = await fetch('https://opentdb.com/api.php?amount=1&type=multiple');
        let trivia = await response.json();
        console.log(trivia);

        let answers = trivia.results[0].incorrect_answers.concat(trivia.results[0].correct_answer);
        shuffle(answers);
        let correct;
        for(let i = 0; i < answers.length; i++){
            if(answers[i] === trivia.results[0].correct_answer){
                switch(i){
                    case 0:
                        correct = 'a';
                        break;
                    case 1:
                        correct = 'b';
                        break;
                    case 2:
                        correct = 'c';
                        break;
                    case 3:
                        correct = 'd';
                        break; 
                }
                break;
            }
        }

        console.log(correct);


        const category = trivia.results[0].category;
        const difficulty = trivia.results[0].difficulty;
        const question = trivia.results[0].question;

        const A = answers[0];
        const B = answers[1];
        const C = answers[2];
        const D = answers[3];

        
        





        const formmated_message = `Question: ${question} 
        Category: ${category}
        Difficulty: ${difficulty}
        A: ${A}
        B: ${B}
        C: ${C}
        D: ${D}`;


        const filter = user_answer =>{
            switch(user_answer.content.toLowerCase()){
                case 'a':
                    return true;
                case 'b':
                    return true;
                case 'c':
                    return true;
                case 'd':
                    return true;
                default:
                    return false;
            }
        };

        function addGold(collected){
            console.log(collected.first().author.id);
            if(players.has(collected.first().author.id)){
                cur = players.get(collected.first().author.id);
                cur.addGold(5);
                message.channel.send(`${collected.first().author} recived 5 gold!`);
            }else{
                message.channel.send(`${collected.first().author} you must join to receive gold`);
            }
        }

        message.channel.send(formmated_message).then(() =>{
            message.channel.awaitMessages(filter, {max: 1, time: 30000, errors: ['time'] })
            .then(collected =>{
                
                if(collected.first().content.toLowerCase() === correct ){
                    message.channel.send(`${collected.first().author} responded correctly`);
                    addGold(collected);
                }else{
                    message.channel.send(`${collected.first().author} responded incorrectly`);
                }
                
            })
            .catch(collected =>{
                message.channel.send("Time is up!");
            })
            .then(()=>{
                message.channel.send(`The correct answer was ${correct.toUpperCase()} = ${trivia.results[0].correct_answer}`);
            })
            
        });

       


    }
}

