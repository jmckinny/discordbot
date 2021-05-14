const fetch = require('node-fetch');
const shuffle = require('shuffle-array');
const html = require('html-entities').AllHtmlEntities;
module.exports = {
    name: 'trivia',
    description: 'answer a trivia question for gold!',
    async execute(message, args, players) {
        let response = await fetch('https://opentdb.com/api.php?amount=1&type=multiple'); //Fetch a question from the database
        let trivia = await response.json();
        console.log(trivia);

        let answers = trivia.results[0].incorrect_answers.concat(trivia.results[0].correct_answer);
        shuffle(answers);

        //This bit finds which of the letter choices corresponds to the correct answer and stores the letter in the "correct" variable
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


        const category = html.decode(trivia.results[0].category);
        const difficulty = html.decode(trivia.results[0].difficulty);
        const question = html.decode(trivia.results[0].question);
        const correct_answer = html.decode(trivia.results[0].correct_answer);

        const A = html.decode(answers[0]);
        const B = html.decode(answers[1]);
        const C = html.decode(answers[2]);
        const D = html.decode(answers[3]);

        const formmated_message = `Question: ${question} 
        Category: ${category}
        Difficulty: ${difficulty}
        A: ${A}
        B: ${B}
        C: ${C}
        D: ${D}`;


        const filter = user_answer =>{ //make sure the collector only gets valid answer choices
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

        function addGold(collected,difficulty_modifier){ //adds the amount of gold to the player given the collected response
            console.log(collected.first().author.id);
            if(players.has(collected.first().author.id)){
                cur = players.get(collected.first().author.id);
                gold_add = 5 * difficulty_modifier;
                cur.addGold(gold_add);
                message.channel.send(`${collected.first().author} recived ${gold_add} gold!`);
            }else{
                message.channel.send(`${collected.first().author} you must join to receive gold`);
            }
        }

        message.channel.send(formmated_message).then(() =>{
            message.channel.awaitMessages(filter, {max: 1, time: 30000, errors: ['time'] }) //Collect messages for 30000ms with a max of one
            .then(collected =>{
                
                if(collected.first().content.toLowerCase() === correct ){ //Check if its the correct letter
                    message.channel.send(`${collected.first().author} responded correctly`);
                    switch(difficulty.toLowerCase()){
                        case 'easy':
                            addGold(collected,1);
                            break;
                        case 'medium':
                            addGold(collected,2);
                            break;
                        case 'hard':
                            addGold(collected,5);
                            break;
                        default: //If the difficulty is unknown????
                            addGold(collected,1);

                    }
                }else{
                    message.channel.send(`${collected.first().author} responded incorrectly`);
                }
                
            })
            .catch(collected =>{ //This catches all errors which is not ideal
                message.channel.send("Time is up!");
            })
            .then(()=>{
                message.channel.send(`The correct answer was ${correct.toUpperCase()} = ${correct_answer}`);
            })
            
        });

       


    }
}

