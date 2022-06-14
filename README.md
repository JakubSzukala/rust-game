# Rust game dev

#### Materials / sources / docs 
- [Game design patterns](http://gameprogrammingpatterns.com/contents.html)


#### Game design patterns simple road map
1. Command pattern

Implementing a generic Command class from which is inherited a specific action 
command, like fireCommand, jumpCommand, so that when a specific command have to
be called, it can be called through parent class. This approach is way more 
flexible, than hard coding some specific commands in for example button handlers.

**I believe that in Rust the inheritance is not really used but still good to 
know such pattern.**

Example use cases:
- Button handlers (easier binding different commands to buttons)
- Implementing undo
- Reduction of coupling or reduction of some global state 
(to command can be passed specific obj / actor to perform the action on)
- From previous point, follows the reusability of the function across different 
entities

Example:

Parent class for generic Command:
```cpp
class Command
{
public:
  virtual ~Command() {}
  virtual void execute() = 0;
};
```

Specific game command:
```cpp
class JumpCommand : public Command
{
public:
  virtual void execute() { jump(); }
};
```

And finally input handler implementation:
```cpp
void InputHandler::handleInput()
{
  if (isPressed(BUTTON_X)) buttonX_->execute();
  else if (isPressed(BUTTON_Y)) buttonY_->execute();
  else if (isPressed(BUTTON_A)) buttonA_->execute();
  else if (isPressed(BUTTON_B)) buttonB_->execute();
}

```
For more details, refer to the [book chapter on Command design pattern](http://gameprogrammingpatterns.com/command.html)

2. asdf dsaf
3. dsfasdf
