const fetch = require('node-fetch');
const shuffle = require('shuffle-array');
module.exports = {
    name: 'trivia',
    descrption: 'answer a trivia question for gold!',
    async execute(message, args, players) {
        let response = await fetch('https://opentdb.com/api.php?amount=1');
        let question = await response.json();
        console.log(question);
        let answers = question.results[0].incorrect_answers.concat(question.results[0].correct_answer);
        shuffle(answers)

        message.channel.send("Category: " + question.results[0].category);
        message.channel.send("Difficulty: " + question.results[0].difficulty);
        message.channel.send("Question = " + question.results[0].question);
        message.channel.send("Answers = " + answers );

    }
}