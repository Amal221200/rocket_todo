export interface TodoProps {
    _id: {
        $oid: string
    },
    body: string,
    completed: boolean
}