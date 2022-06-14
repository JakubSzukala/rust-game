# Rust game dev

#### Materials / sources / docs 
- [Game design patterns](http://gameprogrammingpatterns.com/contents.html)


#### Game design patterns simple road map
1. Command pattern

Implementation of a generic Command class from which is inherited a specific action 
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

2. Flyweight pattern

Data for an object is separated into two kinds:
* specific for an instance
* not specific for an instance

This second kind is then shared among multiple objects to save on memory and 
CPU / GPU cycles when rendering loading etc.

Example use cases:
- Rendering a tree to a scene, not every tree has to have reserved in memory 
tree model, but rather pointer to a tree model that is shared between instances
- Reusability 
- Memory saving 
- Performance improvement
- Making objects more "ligtweight"

Example:
```cpp
class TreeModel
{
private:
  Mesh mesh_;
  Texture bark_;
  Texture leaves_;
};

class Tree
{
private:
  TreeModel* model_;

  Vector position_;
  double height_;
  double thickness_;
  Color barkTint_;
  Color leafTint_;
};

```

For more details, refer to the [book chapter on Flyweight design pattern](http://gameprogrammingpatterns.com/flyweight.html)

3. dsfasdf
