package mlyny.model;

public enum Color {
    NONE(0),
    RED(1),
    BLUE(2);

    public final int value;
    Color(int value) {
        this.value = value;
    }
}
