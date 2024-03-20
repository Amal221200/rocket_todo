// import { useCallback, useState } from "react";
// import { TodoProps } from "../utils/types";
import useSWR from "swr";
import fetcher from "../utils/fetcher";
import { TodoProps } from "../utils/types";
import useSWRMutation from "swr/mutation";
import axios from "axios";

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

    const changeOrder = useSWRMutation(`${import.meta.env.VITE_SERVER_URL}/todo`, async (url, { arg }: { arg: { replacer: TodoProps[] } }) => {
        return axios.patch(`${url}`, { replacer: arg.replacer }).then(res => res.data);
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