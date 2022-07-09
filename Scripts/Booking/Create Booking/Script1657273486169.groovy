import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import static org.assertj.core.api.Assertions.*
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

response = WS.sendRequest(findTestObject('Booking/Create Booking'))

WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

def body = response.getResponseText()
def jsonSlurper = new JsonSlurper()
def jsonResponse = jsonSlurper.parseText(body)
String bookingid = jsonResponse.bookingid
GlobalVariable.bookingid = bookingid

WS.verifyElementPropertyValue(response, 'booking.firstname', 'Boris')
WS.verifyElementPropertyValue(response, 'booking.lastname', 'Johnson')
WS.verifyElementPropertyValue(response, 'booking.totalprice', 150000)
WS.verifyElementPropertyValue(response, 'booking.depositpaid', true)
WS.verifyElementPropertyValue(response, 'booking.bookingdates.checkin', '2022-07-07')
WS.verifyElementPropertyValue(response, 'booking.bookingdates.checkout', '2022-07-08')
WS.verifyElementPropertyValue(response, 'booking.additionalneeds', 'Lunch')