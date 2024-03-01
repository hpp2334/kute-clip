import { bridgeService } from "./bridge.service"
import { RouteKey, routerService } from "./router.service"


export class KeyboardService {
    public initialize() {
        window.addEventListener('keydown', (ev) => {
            const route = routerService.getKey()
            if (route == RouteKey.Main) {
                bridgeService.handle_keydown(ev.code)
            }
        })
    }
}

export const keyboardService = new KeyboardService()
