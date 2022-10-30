package model;

/**
 * handles server messages
 */
public interface IRequestHandler {
    void handle(Object data);
}
