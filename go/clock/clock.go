package clock

import "fmt"

const testVersion = 4

type Clock struct {
    hours int
    minutes int
}

func New(hours, minutes int) Clock {
    hours, minutes = normalize(hours, minutes)
    return Clock { hours, minutes }
}

func (clock Clock) String() string {
    return fmt.Sprintf("%02d:%02d", clock.hours, clock.minutes)
}

func (clock Clock) Add(minutes int) Clock {
    return New(clock.hours, minutes + clock.minutes)
}

func normalize(original_hours, original_minutes int) (int, int) {
    hours := original_hours

    minutes := original_minutes % 60
    if minutes < 0 {
        hours -= 1
        minutes = 60 + minutes
    }

    div_minutes := original_minutes / 60
    hours = (hours + div_minutes) % 24
    if hours < 0 { hours = 24 + hours }

    return hours, minutes
}
