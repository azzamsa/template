describe('Locale Test', () => {
  it('toggle the locale button', () => {
    cy.visit('/about')
    cy.contains('.mx-8 > :nth-child(1)', 'Hi')

    cy.get('.navbar-end > .btn').click()
    cy.contains('.mx-8 > :nth-child(1)', 'Hai')
  })
})
