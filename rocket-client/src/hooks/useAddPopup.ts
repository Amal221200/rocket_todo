import { atom } from "jotai";
import { useAtom } from "jotai/react";
import { useCallback } from "react";

const addPopupAtom = atom(false);

export default function useAddPopup() {
    const [addPopup, setAddPopup] = useAtom(addPopupAtom);

    const onAddPopupOpen = useCallback(() => {
        setAddPopup(true);
    }, [setAddPopup])

    const onAddPopupClose = useCallback((action?: () => Promise<void>) => {
        if (action) {
            return action().then(() => setAddPopup(false))
        }
        setAddPopup(false);
    }, [setAddPopup])

    return {
        addPopup,
        onAddPopupClose,
        onAddPopupOpen
    }
}