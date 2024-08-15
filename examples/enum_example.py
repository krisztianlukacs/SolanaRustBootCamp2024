from enum import Enum

class Color(Enum):
    RED = 1
    GREEN = 2
    BLUE = 3

# Enum értékek használata
print(Color.RED)
print(Color.GREEN)
print(Color.BLUE)

# Enum neveinek és értékeinek elérése
print(Color.RED.name)   # 'RED'
print(Color.RED.value)  # 1

# Enum-ok használata feltételes elágazásban
def describe_color(color):
    if color == Color.RED:
        return "The color is red."
    elif color == Color.GREEN:
        return "The color is green."
    elif color == Color.BLUE:
        return "The color is blue."
    else:
        return "Unknown color."

print(describe_color(Color.RED))
print(describe_color(Color.GREEN))