import http from 'k6/http';
import { check } from 'k6';

export function randomFight() {
    const json_post_header = {
        headers: {
            'Content-Type': 'application/json',
        },
    };
    
    var fight_response = http.post("http://127.0.0.1:9082/random_fight", json_post_header);
    console.log("Fight result:");
    console.log(JSON.stringify(fight_response));
    check(fight_response, {
        'fight result is 200': (r) => r.status === 200
    })
    var fight_result = JSON.parse(fight_response.body);
    return fight_result
}
