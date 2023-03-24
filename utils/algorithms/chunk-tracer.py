import turtle


def get_chunk_tracer(x, z, radius):
    points = (radius * 2) + 2
    is_plus = True
    result = []

    result.append([x, z])

    for index in range(0, points):
        is_plus = not is_plus

        for height in range(0, index):
            if height == points - 2:
                break

            if is_plus:
                z += 1
            else:
                z -= 1

            result.append([x, z])

        if index == points - 1:
            break

        for _ in range(0, index):
            if is_plus:
                x += 1
            else:
                x -= 1

            result.append([x, z])

    return result


if __name__ == "__main__":
    wn = turtle.Screen()
    t = turtle.Turtle()
    t.speed(0)


    result = get_chunk_tracer(0, 0, 8)

    for [x, z] in result:
        print(x, z)

        t.goto(x * 20, z * 20)
        t.dot(10)

    turtle.done()
