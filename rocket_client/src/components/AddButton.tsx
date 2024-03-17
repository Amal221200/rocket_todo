import { Plus } from "lucide-react";
import { ComponentProps, useCallback } from "react";
import cn from "../utils/cn";
import useAddPopup from "../hooks/useAddPopup";

const AddButton = ({ className, ...props }: ComponentProps<'button'>) => {

    const { onAddPopupOpen } = useAddPopup();
    
    const handleClick = useCallback(()=> {
        onAddPopupOpen()
    }, [onAddPopupOpen])
    return (
        <button onClick={handleClick} type="button" {...props} className={cn("bg-teal-500 p-3 rounded", className)}>
            <Plus />
        </button>
    )
}

export default AddButton