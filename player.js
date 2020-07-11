
class Player {
    constructor(name) {
        this.name = name;
        this.gold = 50;
    }

    roll(number) { //Generates a random number from 1 to number
        return Math.ceil(Math.random() * number);
    }

}
module.exports = Player