package mlyny.model;

public enum LColor {
    NONE((byte)0),
    RED((byte)1),
    BLUE((byte)2);

    public final byte value;
    LColor(byte value) {
        this.value = value;
    }

    public static LColor valueOf(byte b) {
        switch(b) {
            case 1: return RED;
            case 2: return BLUE;
            default: return NONE;
        }
    }
}
