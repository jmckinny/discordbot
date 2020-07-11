

function game(player1,player2,bet){
    if(bet > player1.money || bet > player2.money){
        throw "One player does not have the required funds"
    }

    let current = bet*10;
    while(true){
        let p1roll = player1.roll(current)
        console.log(player1.name + " rolled " + p1roll);
        current = p1roll;
        
        if(current == 1){
            return player2.name + " wins!";
        }

        let p2roll = player2.roll(current);
        console.log(player2.name + " rolled " + p2roll);
        current = p2roll;
        
        if(current == 1){
            return player1.name + " wins!";
        }


    }




}





class Player{
    constructor(name){
        this.name = name;
        this.money = 10;
    }

    roll(number){ //Generates a random number from 1 to number
        return Math.ceil(Math.random()*number)
    }

}