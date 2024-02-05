// import validateStrict from './bundle';
// import mandatoryTests from './bundle';
async function validate(doc) {
  await validateStrict(mandatoryTests, doc)
}
 validate("12312312");