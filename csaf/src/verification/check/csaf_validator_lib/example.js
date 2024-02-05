import validateStrict from 'bundle';
import mandatoryTests from 'bundle';
export function myFunction(arg) {
    console.log("Function called with argument:", arg);
    return "Result from JS";
}

async function validate(doc) {
    await validateStrict(mandatoryTests, doc)
}
validate("12312312");