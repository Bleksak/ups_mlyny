package mlyny.controller;

import mlyny.model.Message;

public interface INotifiableController {
    default void receivedMessage(Message message) {}
}
