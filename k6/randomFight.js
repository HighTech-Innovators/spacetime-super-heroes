import http from 'k6/http';
import { check } from 'k6';

export function randomFight() {
    const json_post_header = {
        headers: {
            'Content-Type': 'application/json',
        },
    };
    
    var fight_response = http.post("http://localhost:8082/random_fight", json_post_header);

    check(fight_response, {
        'fight result is 200': (r) => r.status === 200
    })
    var fight_result = JSON.parse(fight_response.body);
    return fight_result
}
