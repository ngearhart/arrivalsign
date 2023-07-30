
interface GenericWidget {
    id?: string
    name: string
    enabled: boolean
}

interface CustomTrain {
    text: string
    target_time: string
    color: string
    show_always: boolean  // If false, wait until it fits naturally on the screen
}

interface DCMetroTrainArrivalWidget extends GenericWidget {
    station_id: string
    custom_trains: CustomTrain[]
}

interface AlertMessage extends GenericWidget {
    text: string
}

interface MessageItem {
    message: string,
    time: Date,
    sticky: boolean
}

export type {
    GenericWidget,
    CustomTrain,
    DCMetroTrainArrivalWidget,
    AlertMessage,
    MessageItem
}
