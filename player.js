class Player {
    constructor(id, name) {
        this.id = id;
        this.name = name;
        this.gold = 50;
        this.collected = new Date();
    }

    roll(number) { //Generates a random number from 1 to number
        return Math.ceil(Math.random() * number);
    }

}