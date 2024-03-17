// import { useCallback, useState } from "react";
// import { TodoProps } from "../utils/types";
import useSWR from "swr";
import fetcher from "../utils/fetcher";
import { TodoProps } from "../utils/types";
import useSWRMutation from "swr/mutation";
import axios from "axios";

export const mockTodos = [
    { _id: { $oid: crypto.randomUUID() }, body: "Create a todo app", completed: false },
    { _id: { $oid: crypto.randomUUID() }, body: "Learn rust", completed: false },
    { _id: { $oid: crypto.randomUUID() }, body: "Understand js", completed: true },
    { _id: { $oid: crypto.randomUUID() }, body: "Create zbpc app", completed: true },
    { _id: { $oid: crypto.randomUUID() }, body: "React 19 release", completed: false },
    { _id: { $oid: crypto.randomUUID() }, body: "Hello World", completed: true },
    { _id: { $oid: crypto.randomUUID() }, body: "Create a todo app", completed: false }
]

export default function useTodo() {
    const { data: todos, mutate: mutateTodo, isLoading } = useSWR<TodoProps[]>(`${import.meta.env.VITE_SERVER_URL}/todo`, fetcher)

    const addTodo = useSWRMutation(`${import.meta.env.VITE_SERVER_URL}/todo`, async (url, { arg }: { arg: { body: string } }) => {
        return axios.post(url, { body: arg.body, completed: false }).then(res => res.data);
    }, {
        onSuccess: () => {
            mutateTodo()
        },
        onError: () => {
            console.log("addTodo error");
        }
    })

    const updateTodo = useSWRMutation(`${import.meta.env.VITE_SERVER_URL}/todo`, async (url, { arg }: { arg: { id: string, data: TodoProps } }) => {
        return axios.put(`${url}/${arg.id}`, arg.data).then(res => res.data);
    }, {
        onSuccess: () => {
            mutateTodo()
        },
        onError: () => {
            console.log("updateTodo error");
        }
    })

    const deleteTodo = useSWRMutation(`${import.meta.env.VITE_SERVER_URL}/todo`, async (url, { arg }: { arg: { id: string } }) => {
        return axios.delete(`${url}/${arg.id}`).then(res => res.data);
    }, {
        onSuccess: () => {
            mutateTodo()
        },
        onError: () => {
            console.log("error");
        }
    })

    const changeOrder = useSWRMutation(`${import.meta.env.VITE_SERVER_URL}/todo`, async (url, { arg }: { arg: { id: string, replaceId: string } }) => {
        return axios.patch(`${url}/${arg.id}`, {replace_id : arg.replaceId}).then(res => res.data);
    }, {
        onSuccess: () => {
            mutateTodo()
        },
        onError: () => {
            console.log("error");
        }
    })

    return { todos, isLoading, addTodo, mutateTodo, updateTodo, deleteTodo, changeOrder }
}