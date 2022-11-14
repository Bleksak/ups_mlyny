package mlyny.model;

public record Player(long id) {
    public Player {
        if(id == 0) {
            throw new IllegalArgumentException("Player ID cannot be zero");
        }
    }

    

}
