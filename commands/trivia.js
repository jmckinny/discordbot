const fetch = require('node-fetch');
const shuffle = require('shuffle-array');
module.exports = {
    name: 'trivia',
    descrption: 'answer a trivia question for gold!',
    async execute(message, args, players) {
        let response = await fetch('https://opentdb.com/api.php?amount=1');
        let trivia = await response.json();
        console.log(trivia);
        let answers = trivia.results[0].incorrect_answers.concat(trivia.results[0].correct_answer);
        shuffle(answers);
        const category = trivia.results[0].category;
        const difficulty = trivia.results[0].difficulty;
        const question = trivia.results[0].question;



        const formmated_message = `Question: ${question} 
        Category: ${category}
        Difficulty: ${difficulty}
        Answers: ${answers}`;

        message.channel.send(formmated_message);
        


        

    }
}