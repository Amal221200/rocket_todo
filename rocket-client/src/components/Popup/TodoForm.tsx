import { ComponentProps, FormEvent, useCallback } from "react"
import cn from "../../utils/cn"
import useAddPopup from "../../hooks/useAddPopup"
import useTodo from "../../hooks/useTodo"
import { toast } from "react-toastify"


const TodoForm = ({ className, ...props }: ComponentProps<'div'>) => {
    const { addPopup, onAddPopupClose } = useAddPopup()
    const { addTodo: { trigger: triggerAdd } } = useTodo();

    const handleCancel = useCallback(() => {
        onAddPopupClose()
    }, [onAddPopupClose])

    const handleSubmit = useCallback((e: FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        const formData = new FormData(e.currentTarget);
        const body = formData.get("body");

        if (!body) {
            return
        }

        const newTodo = { _id: { $oid: crypto.randomUUID() }, body: body.toString().trim(), completed: false }

        onAddPopupClose(async () => {
            triggerAdd({ body: newTodo.body }, {
                optimisticData: (currentData) => {
                    return [...currentData, newTodo]
                }
            });
            toast.success("Created your task")
            e.currentTarget.reset()
        })
    }, [onAddPopupClose, triggerAdd])

    return (
        <div className={cn("flex justify-center items-start absolute transition-all duration-300 -top-full left-0 w-full h-full z-[100] pt-1", addPopup && "top-0", className)}
            {...props}>
            <form onSubmit={handleSubmit} className={cn("min-w-[20vw] transition-transform duration-500 transform ease-in delay-0 -translate-y-[130%] bg-black/50 rounded-md px-2 py-1", addPopup && "duration-500 translate-y-0")}>
                <header>Add Todo</header>
                <div className="space-y-1">
                    <input type="text" name="body" autoFocus={addPopup} className="w-full border-2 px-1 py-[2px] border-gray-700 rounded outline-none" />
                    <div className="flex justify-end gap-3">
                        <button type="submit" className={cn("px-2 py-1 rounded transition-colors", "bg-teal-500", "hover:bg-teal-700")}>Add</button>
                        <button onClick={handleCancel} type="button" className={cn("px-2 py-1 rounded transition-colors", "border-2 border-red-500 text-red-500", "hover:border-transparent hover:text-white hover:bg-red-500")}>Cancel</button>
                    </div>
                </div>
            </form>
        </div>
    )
}

export default TodoForm